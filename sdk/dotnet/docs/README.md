# Rescind.Hook for .NET

A hook is a process that `rescindd` calls on a site's behalf: to record journal
entries, run steps on hosts it cannot reach itself, answer probes, approve
gates, resolve and receive secrets, deliver notifications, or schedule
backstops. The protocol is newline-delimited JSON ([hook-protocol.md]).
This library lets you write a hook as an implementation per kind, and does
the registration handshake, the framing and the reply shapes for you.

- **No dependencies.** .NET 8 or later; `System.Text.Json` is part of the
  platform, so a host application takes on nothing it did not have.
- **Conformance-tested.** `rescind sdk-conform` drives every op of every kind
  through this library's own serve loop ([sdk-conformance.md]).
- **Protocol v1**, which is frozen: a hook written against it keeps working
  until a new protocol version says otherwise.

## Install

The package is `Rescind.Hook`. It is not on nuget.org; reference the project
from a checkout of rescind:

```xml
<ItemGroup>
  <ProjectReference Include="../rescind/sdk/dotnet/src/RueHook/RueHook.csproj" />
</ItemGroup>
```

or pack it (`dotnet pack sdk/dotnet/src/RueHook -c Release -o nupkg`) and
add that directory as a package source.

## Quick start

An audit hook: a journal sink that keeps every entry the engine chains,
and a notifier. This file is `examples/AuditHook/AuditHook.cs`; the
library's test project references it and drives it
(`tests/RueHook.Tests/AuditExampleTests.cs`).

<!-- example: examples/AuditHook/AuditHook.cs -->
```csharp
using System.Text.Json.Nodes;
using Rescind.Hook;

namespace Rescind.Hook.Example;

/// <summary>
/// An audit hook: a journal sink that keeps every entry, and a notifier.
///
/// Bind it in a site with <c>journal to: local(), hook(:audit)</c> and
/// <c>notify via: hook(:audit)</c>, and have rescindd spawn it:
///
///     rescindd run --spawn audit="dotnet /opt/rescind/AuditHook.dll" ...
///
/// Each journal entry is appended to $RESCIND_AUDIT_LOG (default audit.ndjson)
/// as one line of JSON. A sink that cannot record an entry must say so: the
/// engine then refuses to proceed (R0304) rather than run a step nobody
/// recorded. Notifications go to stderr, because stdout carries the protocol.
/// </summary>
public static class AuditHook
{
    private sealed class AuditLog(string path) : IJournal
    {
        public void Append(JsonNode entry)
        {
            try
            {
                File.AppendAllText(path, entry.ToJsonString() + "\n");
            }
            catch (Exception e) when (e is IOException or UnauthorizedAccessException)
            {
                throw new Refusal($"the audit log {path} is not writable: {e.Message}");
            }
        }
    }

    private sealed class Stderr : INotify
    {
        public void Deliver(string level, string subject, string body) =>
            Console.Error.WriteLine($"[{level}] {subject}: {body}");
    }

    public static Hooks Build(string path) => new() { Journal = new AuditLog(path), Notify = new Stderr() };

    public static int Main() =>
        Serve.Stdio("audit", Build(Environment.GetEnvironmentVariable("RESCIND_AUDIT_LOG") ?? "audit.ndjson"));
}
```

The protocol is plain lines, so you can drive the hook by hand. After its
registration line it waits for the acknowledgement, then answers one
request per line:

```text
$ dotnet build examples/AuditHook
$ printf '%s\n' '{"register":{"ok":true}}' \
    '{"id":1,"kind":"journal","op":"append","entry":{"seq":1}}' \
    '{"id":2,"kind":"probe","op":"observe","host":"h","probe":"p"}' |
  dotnet examples/AuditHook/bin/Debug/net8.0/AuditHook.dll
{"register":{"name":"audit","kinds":["journal","notify"],"protocol":1,"filesystem":false,"stdin_preamble":false}}
{"id":1,"ok":true}
{"id":2,"ok":false,"error":"this hook does not serve probe.observe"}
```

## Wire it into a site

The site names the hook where it wants it used, and declares who may
register it:

```text
site do
  journal to: local(), hook(:audit)
  notify via: hook(:audit)
  hooks do
    registrar :spawned, user: :socket_owner, may_register: [:audit]
  end
  ...
end
```

Then `rescindd` starts it as a child and talks to it over its stdin and
stdout:

```sh
rescindd run --site site.scind --store /var/db/rescind --socket /var/run/rescind/rescindd.sock \
  --spawn audit="dotnet /opt/rescind/AuditHook.dll"
```

Four names have to agree: the one the hook registers with (the first
argument of `Serve.Stdio`), the `NAME` of `--spawn NAME=COMMAND`, the
`hook(:audit)` the site binds, and one in a registrar's `may_register`.
`rescindd` refuses a child that registers under any other name. A child `rescindd`
spawned is the socket owner, so its registrar says `user: :socket_owner`.

## Next

- [guide.md](guide.md): every kind and its interface, refusing, secrets,
  and the budget.
- [testing.md](testing.md): testing a hook, and judging it with
  `rescind sdk-conform`.

[hook-protocol.md]: ../../../docs/hook-protocol.md
[sdk-conformance.md]: ../../../docs/sdk-conformance.md
