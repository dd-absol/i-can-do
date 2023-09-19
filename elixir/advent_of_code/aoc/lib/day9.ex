defmodule Day9 do
  def run(input) do
    input
    |> parse()
    |> Enum.scan(List.duplicate({0, 0}, 10), &step/2)
    |> Enum.map(&List.last/1)
    |> Enum.uniq()
    |> Enum.count()
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.flat_map(&parse_line/1)
  end

  def parse_line(line) do
    ~w/#{line}/
    |> List.to_tuple()
    |> (fn {direction, steps} ->
      List.duplicate(
        String.to_atom(direction),
        String.to_integer(steps)
      )
    end).()
  end

  def move(:U, {x, y}), do: {x, y + 1}
  def move(:D, {x, y}), do: {x, y - 1}
  def move(:L, {x, y}), do: {x - 1, y}
  def move(:R, {x, y}), do: {x + 1, y}

  def opposite(:R), do: :L
  def opposite(:L), do: :R
  def opposite(:U), do: :D
  def opposite(:D), do: :U

  def tail_moves({h1, h2}, {t1, t2}) do
    cond do
      h1 - t1 > 1 -> [:R | second_part({h2, t2}, :U)]
      t1 - h1 > 1 -> [:L | second_part({h2, t2}, :U)]
      h2 - t2 > 1 -> [:U | second_part({h1, t1}, :R)]
      t2 - h2 > 1 -> [:D | second_part({h1, t1}, :R)]
      true -> []
    end
  end

  def second_part({hn, tn}, direction) do
    cond do
      hn - tn > 0 -> [direction]
      hn - tn < 0 -> [opposite(direction)]
      true -> []
    end
  end

  def step(direction, rope) do
    rope
    |> List.update_at(0, &(move(direction, &1)))
    |> business_logic()
  end

  def business_logic([h | []]), do: [h |[]]
  def business_logic([h | [t | rest]]) do
    [h | business_logic([tail_moves(h, t) |> Enum.reduce(t, &move/2) | rest])]
  end
end
