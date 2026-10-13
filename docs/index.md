# fire-number-calc

A one-file terminal calculator for the fire number: the portfolio
target your annual spending implies at a given withdrawal rate, the
gap between that target and the balance you hold today, and the
years of saving it takes to close the gap. It prints two timelines,
one for a budget that holds flat until you quit and one for a budget
that creeps up a little every year, and shows what the creep costs in
working years.

fire-number-calc is built and maintained by [Firenomics](https://firenomics.com),
a site that works the same math from the other side: withdrawal-rate
failure points, sequence-of-returns risk, and the expected value of
spending decisions.

## Install

Python 3.8+, standard library only. No install step, no network,
nothing leaves your machine:

    curl -O https://raw.githubusercontent.com/scrapertweeter3-prog/fire-number-calc/main/firenumbercalc.py

## Usage

    $ python3 firenumbercalc.py --spending 52000 --rate 0.035 \
        --balance 310000 --saving 38000 --growth 0.02 --returns 0.055
== fire number
Annual spending                  52,000
Withdrawal rate                   3.50%
Fire number                   1,485,714
Current balance                 310,000
Gap to close                  1,175,714
Years, flat budget                   15
Years, growing budget                20
Extra working years                   5

The printout moves in the order a plan does: the number itself, what
stands between you and it, then the timelines. At 3.5% the target
lands near $1.49 million, and the flat-budget line closes a
$1,175,714 gap in 15 years of $38,000 savings compounding at 5.5%
real.

## Two budgets, two timelines

The flat line holds spending at $52,000 until retirement day. The
growing line lets it rise 2% a year, which raises the target while
the portfolio works, and the tool prices that honestly: 5 extra
years. The arithmetic itself is fixed. The target moves because the
spending it has to cover keeps moving.

## The rate guard

--rate is a rate, not a percent. Passing 4 instead of 0.04 stops
with an argument error instead of printing a target 100 times too
small. The same convention runs through the other flags: --growth
0.02, --returns 0.055.

## JSON output

Add --json for the machine-readable form of the same run:

    $ python3 firenumbercalc.py --spending 52000 --rate 0.035 \
        --balance 310000 --saving 38000 --growth 0.02 --returns 0.055 --json
{
  "fire_number": 1485714.29,
  "gap": 1175714.29,
  "years_flat_budget": "15",
  "years_growing_budget": "20",
  "extra_years_from_growing_budget": "5"
}
