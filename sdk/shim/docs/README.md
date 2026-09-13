# rescind-hook, the shim

A hook is a process that `rescindd` calls on a site's behalf: to record journal
entries, run steps on hosts it cannot reach itself, answer probes, approve
gates, resolve and receive secrets, deliver notifications, or schedule
backstops. The protocol is newline-delimited JSON ([hook-protocol.md]).

`rescind-hook` lets anything that can read a line and write one be a hook: a
shell script, an awk program, a `curl` invocation. It speaks the protocol
to `rescindd` and hands each request to a command you name, on that command's
stdin, taking the reply from its stdout. No SDK, no JSON library, no
handshake.

- **What the shim owns:** the registration frame and its acknowledgement;
  the reply's `id`, which it copies from the request, so a command never
  parses one out or echoes one back; and a command that fails -- a non-zero
  exit, output that is not a JSON object -- becoming `ok: false` with the
  reason rather than a silence.
- **What it deliberately does not do:** repair a reply. The command's
  object goes through as written, so a reply that promises what it does
  not carry is refused by the engine (R0303), visibly, where a shim that
  quietly completed it would hide the mistake.
- **Conformance-tested.** Behind the shim, a POSIX `sh` script passes every
  case of `rescind sdk-conform` ([sdk-conformance.md]).

## Install

`rescind-hook` is built and released beside `rescind` and `rescindd`, for the same
targets. From a checkout of rescind:

```sh
cargo install --path sdk/shim
```

## Quick start

An audit hook: a journal sink that keeps every entry the engine chains,
and a notifier, in POSIX sh. This file is `examples/audit.sh`; the shim's
own tests run it behind the shim (`tests/audit_example.rs`).

<!-- example: examples/audit.sh -->
```sh
#!/bin/sh
# An audit hook behind the rescind-hook shim: a journal sink that keeps every
# entry, and a notifier, in POSIX sh with no JSON parser.
#
#   rescindd run --spawn audit="rescind-hook --name audit --kinds journal,notify \
#                             --command /usr/local/libexec/audit.sh" ...
#
# The shim hands each request to this script as one line of JSON on its
# stdin, and puts the request's id on the reply itself. A journal request
# is appended to $RESCIND_AUDIT_LOG (default audit.ndjson) whole, since it
# carries the entry. A sink that cannot record an entry must say so: the
# engine then refuses to proceed (R0304) rather than run a step nobody
# recorded. Notifications go to stderr, because stdout carries the reply.
set -u
read -r req || exit 0

case $req in
  *'"kind":"journal"'*)
    if { printf '%s\n' "$req" >> "${RESCIND_AUDIT_LOG:-audit.ndjson}"; } 2> /dev/null; then
      printf '{"ok":true}\n'
    else
      printf '{"ok":false,"error":"the audit log is not writable"}\n'
    fi ;;
  *'"kind":"notify"'*)
    printf '%s\n' "$req" >&2
    printf '{"ok":true}\n' ;;
  *)
    printf '{"ok":false,"error":"this hook serves journal and notify only"}\n' ;;
esac
```

Drive it by hand through the shim. After the registration line the shim
waits for the acknowledgement, then hands each request to the script:

```text
$ printf '%s\n' '{"register":{"ok":true}}' \
    '{"id":1,"kind":"journal","op":"append","entry":{"seq":1}}' \
    '{"id":2,"kind":"probe","op":"observe","host":"h","probe":"p"}' |
  rescind-hook --name audit --kinds journal,notify --command "sh examples/audit.sh"
{"register":{"filesystem":false,"kinds":["journal","notify"],"name":"audit","protocol":1,"stdin_preamble":false}}
{"id":1,"ok":true}
{"error":"this hook serves journal and notify only","id":2,"ok":false}
```

The engine never sends a kind the hook did not register; the script's own
refusal of `probe` above is what `rescind sdk-conform` checks when it sends an
op the protocol does not have.

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

Then `rescindd` starts the shim as a child:

```sh
rescindd run --site site.scind --store /var/db/rescind --socket /var/run/rescind/rescindd.sock \
  --spawn audit="rescind-hook --name audit --kinds journal,notify --command /usr/local/libexec/audit.sh"
```

Four names have to agree: the shim's `--name`, the `NAME` of `--spawn
NAME=COMMAND`, the `hook(:audit)` the site binds, and one in a registrar's
`may_register`. `rescindd` refuses a child that registers under any other name.
A child `rescindd` spawned is the socket owner, so its registrar says `user:
:socket_owner`. A hook the journal or the inventory depends on must be
spawned this way: the daemon needs it before it starts listening.

## Next

- [guide.md](guide.md): the command's contract, the reply each kind must
  give, and secrets.
- [testing.md](testing.md): testing a command, and judging it with
  `rescind sdk-conform`.

[hook-protocol.md]: ../../../docs/hook-protocol.md
[sdk-conformance.md]: ../../../docs/sdk-conformance.md
