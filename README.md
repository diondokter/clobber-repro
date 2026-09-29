Reproduces on 1.59 (introduction of asm!) to latest stable (1.98 as of writing) and the most recent nightly.

Weird warning on this inline asm for thumb targets.
Afaik, these targets don't have these high D16+ registers.
Remove the `hf` (hard-float) from the target and the warning disappears.

```
cargo b
   Compiling clobber-repro v0.1.0 (R:\clobber-repro)
warning: inline asm clobber list contains reserved registers: D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31
  --> src\main.rs:10:14
   |
10 |             "mov r0, {exp}",
   |              ^^^^^^^^^^^^^

note: Reserved registers on the clobber list may not be preserved across the asm statement, and clobbering them may lead to undefined behaviour.
  --> src\main.rs:10:14
   |
10 |             "mov r0, {exp}",
   |              ^^^^^^^^^^^^^
```