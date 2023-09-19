defmodule Day17.Parse do
  def parse(input) do
    input
    |> String.codepoints()
    |> Enum.filter(&(&1 != "\n"))
    |> Enum.map(fn
      ">" -> :right 
      "<" -> :left
    end)
  end
end
