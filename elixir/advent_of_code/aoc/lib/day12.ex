defmodule Day12 do
  def run(input) do
    map = parse(input)

    {s1, s2} = map |> find_in_map("S" |> String.to_charlist() |> hd)
    {e1, e2} = map |> find_in_map("E" |> String.to_charlist() |> hd)

    map = map
    |> update_in([Access.at(s1), Access.at(s2)], fn _ -> "a" |> String.to_charlist |> hd end)
    |> update_in([Access.at(e1), Access.at(e2)], fn _ -> "z" |> String.to_charlist |> hd end)

    business_logic([%{position: {e1, e2}, count: 0, map: map}], [])
  end

  def find_in_map(map, char) do
    map
    |> Enum.with_index()
    |> Enum.filter(fn {line, _} -> char in line end)
    |> hd()
    |> then(fn {line, i} ->
      {i, line |> Enum.find_index(&(&1 == char))}
    end)
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&String.to_charlist/1)
  end

  @directions [:up, :down, :left, :right]

  def move(:up, {x, y}), do: {x, y + 1}
  def move(:down, {x, y}), do: {x, y - 1}
  def move(:left, {x, y}), do: {x - 1, y}
  def move(:right, {x, y}), do: {x + 1, y}

  def read_map(map, {x, y}) do
    map |> get_in([Access.at(x), Access.at(y)])
  end

  def business_logic([state | queue], explored) do
    if state.map|> read_map(state.position) == "a" |> String.to_charlist() |> hd do
      state.count
    else
      state
      |> children()
      |> Enum.filter(&(!(&1.position in explored)))
      |> then(fn children ->
        (business_logic(queue ++ children, ((children |> Enum.map(&(Map.get(&1, :position))) ) ++ explored) |> Enum.uniq()))
      end)
    end
  end

  def children(state) do
    @directions
    |> Enum.map(&(move(&1, state.position)))
    |> Enum.filter(&(read_map(state.map, &1)))
    |> Enum.filter(fn next_pos ->
      state.map
      |> then(&(-(read_map(&1, next_pos) - read_map(&1, state.position)) <= 1))
    end)
    |> Enum.map(&(%{%{state | count: state.count + 1} | position: &1}))
  end
end
