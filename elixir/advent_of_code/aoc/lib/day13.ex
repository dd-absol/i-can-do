defmodule Day13 do
  alias Day13.Parse

  def run(input) do
    input
    |> Parse.parse()
    |> Enum.sort(&right_order?/2)
    |> Enum.with_index(1)
    |> Enum.filter(fn {packet, _} -> packet == [[6]] || packet == [[2]] end)
    |> Enum.map(&(&1 |> elem(1)))
    |> Enum.product()
  end

  defp right_order?(x1, x2), do: right_order?({x1, x2})
  defp right_order?({[], []}), do: :non_conclusive
  defp right_order?({[], _}), do: true
  defp right_order?({_, []}), do: false
  defp right_order?({[h1 | t1], [h2 | t2]}) do
    if [h1, h2] |> Enum.all?(&is_integer/1) do
      cond do
        h1 < h2 -> true
        h1 > h2 -> false
        true -> right_order?({t1, t2})
      end
    else
      case [h1, h2]
        |> Enum.map(fn h ->
          if is_integer(h) do
            [h]
          else
            h
          end
        end)
        |> List.to_tuple()
        |> right_order?()
      do
        :non_conclusive -> right_order?({t1, t2})
        b -> b
      end
    end
  end
end
