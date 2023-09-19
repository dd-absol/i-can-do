defmodule Day8 do
  def run(input) do
    forest = parse(input)
    reverse_order_rows = fn rows -> rows |> Enum.map(&Enum.reverse/1) end
    [east, north, west, south] = [
      &Function.identity/1,
      &swap_rows_cols/1,
      reverse_order_rows,
      &(&1 |> swap_rows_cols |> reverse_order_rows.())
    ]
    |> Enum.map(fn f ->
      forest
      |> f.()
    end)

    n = length(forest)
    forest
    |> List.flatten()
    |> Enum.map(fn %{position: {i, j}, height: h} ->
      [
        east  |> Enum.at(i) |> Enum.map(&(&1.height)) |> Enum.drop(j + 1),
        north |> Enum.at(j) |> Enum.map(&(&1.height)) |> Enum.drop(i + 1),
        west  |> Enum.at(i) |> Enum.map(&(&1.height)) |> Enum.drop(n - j),
        south |> Enum.at(j) |> Enum.map(&(&1.height)) |> Enum.drop(n - i)
      ]
      |> Enum.map(&(business_logic(&1, h)))
      |> Enum.product()
    end)
    |> Enum.max()
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.with_index()
    |> Enum.map(fn {line, i} ->
      line
      |> String.graphemes
      |> Enum.with_index()
      |> Enum.map(fn {char, j} ->
        %{
          position: {i, j},
          height: String.to_integer(char)
        }
      end)
    end)
  end

  def business_logic(line, height) do
      res = line
        |> Enum.take_while(&(&1 < height))
        |> Enum.count()
      if res == length(line) do
        res
      else
        res + 1
      end
  end

  defp swap_rows_cols( [h|_t] ) when h==[], do: []
  defp swap_rows_cols(rows) do
    firsts = Enum.map(rows, fn(x) -> hd(x) end) # first element of each row
    rest = Enum.map(rows, fn(x) -> tl(x) end)   # remaining elements of each row
    [firsts | swap_rows_cols(rest)]
  end
end
