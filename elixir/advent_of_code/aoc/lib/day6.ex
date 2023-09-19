defmodule Day6 do
  def run(input) do
    input
    |> String.to_charlist()
    |> Enum.chunk_every(14, 1)
    |> Enum.with_index()
    |> Enum.filter(fn {seq, _} ->
      14 == Enum.uniq(seq) |> length()
    end)
    |> hd
    |> elem(1)
  end
end
