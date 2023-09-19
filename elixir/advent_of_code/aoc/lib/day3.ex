defmodule Day3 do
  def run(input) do
    input
    |> parse
    |> Enum.map(&business_logic/1)
    |> Enum.map(&value/1)
    |> Enum.sum()

  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.chunk_every(3, 3, :discard)
  end

  def business_logic(list) do
    conversion = fn str ->
      str |> String.to_charlist() |> MapSet.new()
    end
    list
    |> Enum.map(conversion)
    |> Enum.reduce(&MapSet.intersection/2)
    |> MapSet.to_list()
    |> hd()
  end

  def value(char) do
    offset = if char > 96 do 96 else (64 - 26) end
    char - offset
  end
end
