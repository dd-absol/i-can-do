defmodule Day4 do
  def run(input) do
    input
    |> parse
    |> Enum.filter(&(!business_logic(&1)))
    |> Enum.count()
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_line/1)
  end

  def parse_line(line) do
    line
    |> String.split(",")
    |> Enum.map(fn
      line -> line
      |> String.split("-")
      |> Enum.map(&String.to_integer/1)
    end)
  end

  def business_logic([r1, r2]) do
    r1
    |> to_range()
    |> Range.disjoint?(to_range(r2))
  end

  def contains([a, b], [c, d]) do
    a <= c && d <= b
  end

  def to_range([a, b]) do
    Range.new(a, b)
  end
end
