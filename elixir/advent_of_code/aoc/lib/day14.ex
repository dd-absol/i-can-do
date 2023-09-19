defmodule Day14 do
  alias Day14.Parse

  @x_max 1000

  def run(input) do
    Enum.reduce_while(1..1000000, build_map(input), &business_logic/2)
  end

  defp build_map(input) do
    input
    |> Parse.parse()
    |> Enum.flat_map(&(&1 |> Enum.chunk_every(2, 1, :discard)))
    |> Enum.flat_map(&build_line/1)
    |> Enum.uniq()
    |> lay_on_map()
  end

  defp lay_on_map(point_list) do
    y_max = point_list
    |> Enum.map(&(&1 |> elem(1)))
    |> Enum.max()
    |> (&(&1 + 2)).()

    point_list |> Enum.reduce(List.duplicate(List.duplicate(true, y_max), @x_max), &take/2)
  end

  defp build_line([{x1, y1}, {x2, y2}]) do
    if x1 == x2 do
      y1..y2 |> Enum.map(&({x1, &1}))
    else
      x1..x2 |> Enum.map(&({&1, y1}))
    end
  end

  defp drop_from({x, y}, map) do
    cond do
      is_free?({x, y + 1}, map) -> drop_from({x, y + 1}, map)
      is_free?({x - 1, y + 1}, map) -> drop_from({x - 1, y + 1}, map)
      is_free?({x + 1, y + 1}, map) -> drop_from({x + 1, y + 1}, map)
      true -> {x, y}
    end
  end

  defp business_logic(count, map) do
    case drop_from({500, 0}, map) do
      {500, 0} -> {:halt, count}
      coords -> {:cont, take(coords, map)}
    end
  end

  defp take({x, y}, map) do
    map |> put_in([x, y] |> Enum.map(&Access.at/1), false)
  end

  defp is_free?({x, y}, map) do
    get_in(map, [x, y] |> Enum.map(&Access.at/1))
  end
end
