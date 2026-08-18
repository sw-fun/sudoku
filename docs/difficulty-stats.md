# Difficulty Statistics Evidence

Measured on 2026-08-18 with `cargo test -p suduko-engine --test
statistical_gates` seeds (base 10000, level offsets 0/100/200/300/400) plus a
10-sample-per-level harvest run on the same engine revision. All runs are
seeded and deterministic; re-running reproduces this table exactly.

| Level   | Score band   | Mean | Min | Max | Clues | 10-sample wall |
|---------|--------------|------|-----|-----|-------|----------------|
| easy    | 100-299      | 147  | 137 | 237 | 44    | 14 ms          |
| medium  | 300-399      | 356  | 354 | 360 | 28    | 763 ms         |
| hard    | 400-599      | 469  | 456 | 565 | 26-27 | 1620 ms        |
| harder  | 600-799      | 660  | 656 | 668 | 26-27 | 1185 ms        |
| hardest | 800+         | 924  | 912 | 941 | 24-25 | 470 ms         |

Findings:

- Mean scores strictly increase across levels (147 < 356 < 469 < 660 < 924),
  so an easy board is never harder than a hard one at the sampled operating
  point.
- Every sampled puzzle has exactly one solution (capped counter at 2) and a
  hardest technique inside its band; every easy sample used naked or hidden
  singles only; every hardest sample required trial.
- Score bands are disjoint by construction (score is hardest-technique
  weight times 100 plus application count), so mean separation follows from
  band membership; the gates prove generation reaches each band at the
  sampled seeds and stays unique.
- Generation cost stays interactive: worst observed level (hard) averaged
  162 ms per accepted puzzle in this run; the acceptance loop caps at 24
  digs x 3 grids before failing closed.

Methodology and limitations: samples are fixed-seed point checks, not
distributional claims over all seeds. Yield estimates that informed the
band design (about 46 percent singles, 16 percent locked, 5 percent
subsets, 6 percent wings, 26 percent trial at dig target 26-28) came from
80-480 seed scans recorded in the step 005-007 commit messages.
