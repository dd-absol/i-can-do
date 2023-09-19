defmodule Day5 do
  def run(input) do
    {state, instructions} = parse(input)
    instructions
    |> Enum.reduce(state, &business_logic/2)
    |> Enum.map(&hd/1)
  end

  def parse(input) do
    with [crates, instructions] <- String.split(input, "\n\n") do
      {
        parse_crates(crates),
        instructions
        |> String.split("\n")
        |> Enum.map(&parse_instruction/1)
      }
    end
  end

  def parse_crates(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_crate_line/1)
    |> Enum.reverse()
    |> tl
    |> Enum.reduce(List.duplicate([], 9), &parse_crates_aux/2)
  end

  def parse_crates_aux(elem, acc) do
    Enum.zip_with(
      acc,
      elem,
      &(if &2 != "   "do
        [&2 | &1]
      else
        &1
      end)
    )
  end

  def parse_crate_line(line) do
    line
    |> String.to_charlist()
    |> Enum.chunk_every(3, 4)
    |> Enum.map(&List.to_string/1)
  end

  def parse_instruction(input) do
    words = ~w/#{input}/
    quantity = String.to_integer(Enum.at(words, 1))
    start = String.to_integer(Enum.at(words, 3)) - 1
    finish = String.to_integer(Enum.at(words, 5)) - 1
    %{
      quantity: quantity,
      start: start,
      finish: finish
    }
  end

  def business_logic(instruction, state) do
    state
    |> Enum.with_index()
    |> Enum.map(fn {stack, i} ->
      if i == instruction.finish do
        {(Enum.at(state, instruction.start) |> Enum.take(instruction.quantity)) ++ stack, i}
      else
        {stack, i}
      end
    end)
    |> Enum.map(fn {stack, i} ->
      if i == instruction.start do
        stack |> Enum.drop(instruction.quantity)
      else
        stack
      end
    end)
  end
end
