defmodule Day10 do
  def run(input) do
    input
    |> parse()
    |> Enum.scan(1, &cycle/2)
    |> then(&([1 | &1]))
    |> Enum.with_index()
    |> Enum.map(&business_logic/1)
    |> Enum.chunk_every(40)
    |> Enum.map(&List.to_string/1)
    |> Enum.join("\n")
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.flat_map(&parse_line/1)
  end

  def parse_line(line) do
    case ~w/#{line}/ do
      ["addx", n] -> [:noop, String.to_integer(n)]
      _ -> [:noop]
    end
  end

  def cycle(instruction, register) do
    case instruction do
      :noop -> register
      n -> register + n
    end
  end

  def business_logic({register, cycle}) do
    if abs(rem(cycle, 40) - register) < 2 do
      "#"
    else
      "."
    end
  end
end
