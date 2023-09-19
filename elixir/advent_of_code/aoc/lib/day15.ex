defmodule Day15 do
  @max_coord 4000000

  def run(input) do
    input
    |> Day15.Parse.parse()
    |> Enum.flat_map(&business_logic/1)
    |> Enum.filter(fn {y, _} -> y >= 0 && y <= @max_coord end)
    |> Enum.group_by(&(&1 |> elem(0)), &(&1 |> elem(1)))
    |> tap(&IO.inspect/1)
    |> Enum.map(&combine_v2/1)
    |> Enum.filter(&(!Enum.empty?(&1 |> elem(1))))
    |> hd()
  end

  defp combine_v2({y, ranges}) do
    {
      y,
      ranges
      |> Enum.map(&MapSet.new/1)
      |> Enum.reduce(MapSet.new(0..@max_coord), &(MapSet.difference(&2, &1)))
      |> Enum.to_list()
    }
  end

  def has_room?(ranges) do
    ranges
    |> Enum.count()
    |> then(&(&1 < @max_coord + 1))
  end

  defp manhattan_distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end

  defp build_ranges({s1, s2}, max_dist) do
    0..max_dist
    |> Enum.flat_map(fn n ->
      [
        {s2 + max_dist - n, (s1 - n)..(s1 + n)},
        {s2 - max_dist + n, (s1 - n)..(s1 + n)}
      ]
    end)
  end

  defp business_logic(line) do
    line.sensor
    |> build_ranges(manhattan_distance(line.sensor, line.beacon))
  end

end
