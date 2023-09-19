defmodule Day11 do
  def run(input) do
    monkeys = input
    |> parse()

    ppcm = monkeys
    |> Enum.map(&(get_in(&1, [:test, :divisor])))
    |> Enum.product()

    1..10000
    |> Enum.reduce(monkeys, fn _, acc -> full_round(acc, ppcm) end)
    |> Enum.map(&(Map.get(&1, :count)))
    |> Enum.sort(:desc)
    |> Enum.take(2)
    |> Enum.product()
  end

  def parse(input) do
    input
    |> String.split("\n\n")
    |> Enum.map(&parse_monkey/1)
  end

  def parse_monkey(monkey) do
    lines = monkey
    |> String.split("\n")
    |> Enum.map(&(~w/#{&1}/))

    %{
      id: lines |> Enum.at(0) |> List.last() |> Integer.parse() |> elem(0),
      items: lines |> Enum.at(1) |> Enum.drop(2) |> Enum.map(&(&1 |> Integer.parse() |> elem(0))),
      op: lines |> Enum.at(2) |> parse_op(),
      test: %{divisor: lines |> Enum.at(3) |> parse_last(),
        success: lines |> Enum.at(4) |> parse_last(),
        failure: lines |> Enum.at(5) |> parse_last()
      },
      count: 0
    }
  end

  def parse_last(line) do
    line |> List.last() |> String.to_integer()
  end

  def parse_op(line) do
    line
    |> Enum.drop(4)
    |> List.update_at(1, &(case Integer.parse(&1) do
      {n, _} -> n
      _ -> :old
    end))
    |> List.update_at(0, fn op ->
      case op do
        "*" -> :times
        "+" -> :plus
      end
    end)
    |> List.to_tuple()
  end

  def full_round(monkeys, ppcm) do
    monkeys |> Enum.map(&(Map.get(&1, :id))) |> Enum.reduce(monkeys, &(turn(&1, &2, ppcm)))
  end

  def turn(monkey_id, monkeys, ppcm) do
    monkey = monkeys |> Enum.at(monkey_id)

    monkey.items
    |> Enum.map(&(compute(&1, monkey.op, ppcm)))
    |> Enum.reduce(
      monkeys |> List.replace_at(monkey_id, %{%{monkey | items: []} | count: (length(monkey.items) + monkey.count)}),
      &(throw(&1, &2, monkey))
    )
  end

  def compute(worry, {op, n}, ppcm) do
    modifier = case n do
      :old -> worry
      _ -> n
    end

    rem(case op do
      :times -> worry * modifier
      :plus -> worry + modifier
    end, ppcm)
  end

  def test(worry, test) do
    if rem(worry, test.divisor) == 0 do
      test.success
    else
      test.failure
    end
  end

  def throw(worry, monkeys, monkey) do
    update_in(
      monkeys,
      [Access.at(test(worry, monkey.test)), :items],
      &(&1 ++ [worry])
    )
  end
end
