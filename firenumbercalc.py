#!/usr/bin/env python3
"""firenumbercalc.py - fire number math for early retirement: the target
balance from annual spending and withdrawal rate, the gap from your current
balance, and both timelines when the budget grows before you quit.

The 4% shortcut says your number is spending times 25. The part people
skip is the second line: what that number means from where you actually
stand, in years, at your real savings rate. This tool prints both, and
prints what a growing budget does to the timeline.

Stdlib only. Exit code 0 always (informational tool).
"""

import argparse
import json
import sys


def _positive(text):
    try:
        val = float(text)
    except ValueError:
        raise argparse.ArgumentTypeError(f"not a number: {text!r}")
    if val < 0:
        raise argparse.ArgumentTypeError(f"must be >= 0, got {val}")
    return val


def fire_number(annual_spending, withdrawal_rate):
    return annual_spending / withdrawal_rate if withdrawal_rate > 0 else 0.0


def gap_to_target(current_balance, target):
    return max(target - current_balance, 0.0)


def years_to_target(current_balance, annual_saving, target, return_rate, spending_growth):
    """Whole years until the portfolio covers the target. The target grows
    at spending_growth a year; the portfolio grows at return_rate with
    savings added annually."""
    if current_balance >= target:
        return 0.0
    for n in range(1, 601):
        grow = (1.0 + return_rate) ** n
        bal = current_balance * grow
        if return_rate > 0:
            bal += annual_saving * (grow - 1.0) / return_rate
        else:
            bal += annual_saving * n
        tgt = target * (1.0 + spending_growth) ** n
        if bal >= tgt:
            return float(n)
    return float("inf")


def budget_cost(current_balance, annual_saving, target, return_rate, spending_growth):
    flat = years_to_target(current_balance, annual_saving, target, return_rate, 0.0)
    grown = years_to_target(current_balance, annual_saving, target, return_rate, spending_growth)
    return flat, grown


def main():
    p = argparse.ArgumentParser(description="fire number math from spending and withdrawal rate")
    p.add_argument("--spending", type=_positive, required=True, help="annual spending in dollars")
    p.add_argument("--rate", type=_positive, required=True, help="withdrawal rate, 0.04 = 4%%")
    p.add_argument("--balance", type=_positive, default=0.0, help="current portfolio balance")
    p.add_argument("--saving", type=_positive, default=0.0, help="annual savings added to the portfolio")
    p.add_argument("--growth", type=_positive, default=0.0, help="annual spending growth until retirement, 0.02 = 2%%")
    p.add_argument("--returns", type=_positive, default=0.0, help="expected real return, 0.06 = 6%%")
    p.add_argument("--json", action="store_true", help="machine output")
    args = p.parse_args()

    if args.rate > 0.25:
        p.error("--rate is a withdrawal rate, 0.04 = 4%, not 4")

    target = fire_number(args.spending, args.rate)
    gap = gap_to_target(args.balance, target)
    flat, grown = budget_cost(args.balance, args.saving, target, args.returns, args.growth)

    def fmt_years(y):
        return "never" if y == float("inf") else f"{y:.0f}"

    out = {
        "fire_number": round(target, 2),
        "gap": round(gap, 2),
        "years_flat_budget": fmt_years(flat),
        "years_growing_budget": fmt_years(grown),
        "extra_years_from_growing_budget": fmt_years(max(grown - flat, 0.0)),
    }
    if args.json:
        print(json.dumps(out, indent=2))
        return

    print("== fire number")
    print(f"Annual spending            {args.spending:>12,.0f}")
    print(f"Withdrawal rate            {args.rate:>12.2%}")
    print(f"Fire number                {target:>12,.0f}")
    print(f"Current balance            {args.balance:>12,.0f}")
    print(f"Gap to close               {gap:>12,.0f}")
    if args.saving > 0:
        print(f"Years, flat budget         {fmt_years(flat):>12}")
        if args.growth > 0:
            print(f"Years, growing budget      {fmt_years(grown):>12}")
            print(f"Extra working years        {fmt_years(max(grown - flat, 0.0)):>12}")


if __name__ == "__main__":
    main()
