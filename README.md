# Fire-number-calc - fire number math for early retirement

A terminal calculator for the number every early retirement plan hangs on:
annual spending divided by your withdrawal rate. It prints the target
balance, the gap from where you stand today, the years of saving it takes
to close that gap, and what a spending budget that grows before you quit
does to the timeline.

Python 3.8+, standard library only. No install, no network, nothing leaves
your machine.

## The method

- **Fire number** = annual spending / withdrawal rate. $40,000 of spending
  at 4% is a $1,000,000 target.
- **Gap** = target minus current balance, floored at zero.
- **Both timelines** = whole years until the portfolio covers the number.
  The flat-budget line assumes the $40,000 never moves. The growing-budget
  line lets spending creep 2% a year until you quit, so the target itself
  rises while the portfolio compounds; the tool prints both and the extra
  working years between them.

## Usage

```
$ python3 firenumbercalc.py --spending 40000 --rate 0.04 \
    --balance 620000 --saving 24000 --growth 0.02 --returns 0.06
== fire number
Annual spending                  40,000
Withdrawal rate                   4.00%
Fire number                   1,000,000
Current balance                 620,000
Gap to close                    380,000
Years, flat budget                    6
Years, growing budget                 8
Extra working years                   2
```

`--json` for machine output.

## Honest limits

The tool treats the withdrawal rate as a fixed input and computes in whole
years: it does not model sequence-of-returns risk, taxes, Social Security
timing, or the spending pilot that would test the budget assumption
itself. If the budget line is wrong, every other line is wrong with it,
so check the spending number first.

## Related

Testing the spending assumption itself with a pre-retirement pilot:
https://firenomics.com/lifestyle/fire-number-spending-pilot/
Where the crossover math comes from and when it lands:
https://firenomics.com/investing/fire-crossover-point/
More fire math: https://firenomics.com/

## License

MIT
