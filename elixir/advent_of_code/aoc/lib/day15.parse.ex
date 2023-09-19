defmodule Day15.Parse do
  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_line/1)
  end

  defp parse_line(line) do
    words = ~w/#{line}/

    %{
      sensor: {
        words |> Enum.at(2) |> parse_word(),
        words |> Enum.at(3) |> parse_word()
      },
      beacon: {
        words |> Enum.at(8) |> parse_word(),
        words |> Enum.at(9) |> parse_word()
      }
    }
  end

  defp parse_word(word) do
    word
    |> String.split_at(2)
    |> elem(1)
    |> Integer.parse()
    |> elem(0)
  end
end
