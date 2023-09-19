defmodule Day17 do
  @rock_nb 100000
  
  def run(input) do
    jets = input
    |> Day17.Parse.parse()
    |> Enum.into(Arrays.new())
    
    Stream.cycle([:horizontal, :plus, :reverse_l, :vertical, :square])
    |> Enum.take(@rock_nb)
    |> Enum.reduce(%{
      map: MapSet.new(),
      jets: {jets, 0},
    }, &drop/2)
    |> Map.get(:map)
    |> max_height()
  end

  defp next_jet(jets, index) do
    jets |> Arrays.get(rem(index, Arrays.size(jets)))
  end

  defp rock(:reverse_l), do: fn {x, y} -> [{x, y}, {x + 1, y}, {x + 2, y}, {x + 2, y + 1}, {x + 2, y + 2}] end
  defp rock(:horizontal), do: fn {x, y} -> 0..3 |> Enum.map(&{x + &1, y}) end
  defp rock(:vertical), do: fn {x, y} -> 0..3 |> Enum.map(&{x, y + &1}) end
  defp rock(:square), do: fn {x, y} -> [{x, y}, {x + 1, y}, {x, y + 1}, {x + 1, y + 1}] end
  defp rock(:plus), do: fn {x, y} -> [{x + 1, y}, {x + 1, y + 1}, {x + 1, y + 2}, {x, y + 1}, {x + 2,  y + 1}] end

  defp can_move?(rock, {x, y}, map, direction) do
    !(rock(rock).({x, y})
    |> Enum.map(&(move(&1, direction)))
    |> Enum.any?(fn {a, b} -> b < 1 || a < 0 || a > 6 || MapSet.member?(map, {a, b}) end))
  end

  defp move({x, y}, :left), do: {x - 1, y}
  defp move({x, y}, :right), do: {x + 1, y}
  defp move({x, y}, :down), do: {x, y - 1}
    
  defp max_height(map) do
    map
    |> Enum.map(&(elem(&1, 1)))
    |> Enum.max(&>=/2, fn -> 0 end) 
  end

  defp drop(rock, state) do
    step(rock, state, {2, max_height(state.map) + 4}) |> tap(&IO.inspect(&1.jets |> elem(1))) 
  end
  
  defp step(rock, state, position) do
    {jet, state} = state |> get_and_update_in([:jets], fn {jets, index} -> {next_jet(jets, index), {jets, index + 1}} end)

    new_position = if can_move?(rock, position, state.map, jet) do
     move(position, jet)
    else
      position
    end

    if can_move?(rock, new_position, state.map, :down) do
      step(rock, state, move(new_position, :down))
    else
      state |> update_in([:map], &(place_rock(&1, rock, new_position)))
    end
  end

  defp place_rock(map, rock, position) do
    rock(rock).(position)
    |> Enum.reduce(map, &MapSet.put(&2, &1))
  end
end
