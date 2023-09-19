defmodule Day13.Parse do
  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&unwrap_list/1)
    |> Enum.map(&parse_packet/1)
  end

  defp parse_packet(packet) do
    case packet |> String.next_codepoint() do
      nil -> []
      {"[", tail} ->
        {list, tail} = parse_list(tail, [], 0)
        [parse_packet(list) | parse_packet(tail)]
      {",", tail} -> parse_packet(tail)
      {" ", tail} -> parse_packet(tail)
      {_, _} ->
        {n, tail} = parse_number(packet, [])
        [n | parse_packet(tail)]
    end
  end

  defp parse_list(packet, acc, depth) do
    case packet |> String.next_codepoint() do
      {"]", tail} when depth == 0 -> {acc |> Enum.reverse() |> List.to_string(), tail}
      {"[", tail} -> parse_list(tail, ["[" | acc], depth + 1)
      {"]", tail} -> parse_list(tail, ["]" | acc], depth - 1)
      {c, tail} -> parse_list(tail, [c | acc], depth)
    end
  end

  defp parse_number(packet, acc) do
    case packet |> String.next_codepoint() do
      nil -> {acc |> Enum.reverse() |> List.to_string() |> String.to_integer(), ""}
      {",", tail} -> {acc |> Enum.reverse() |> List.to_string() |> String.to_integer(), tail}
      {c, tail} -> parse_number(tail, [c | acc])
    end
  end

  defp unwrap_list(data) do
    data |> String.slice(1, (String.length(data) - 2))
  end
end
