defmodule Day14.Parse do
  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_line/1)
  end

  defp parse_line(line) do
    line
    |> String.split("->", trim: true)
    |> Enum.map(&parse_coordinates/1)
  end

  defp parse_coordinates(coords) do
    coords
    |> String.split(",")
    |> Enum.map(&(&1 |> String.trim() |> String.to_integer()))
    |> List.to_tuple()
  end
end
