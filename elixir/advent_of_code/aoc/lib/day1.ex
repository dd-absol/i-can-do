defmodule Day1 do
  @moduledoc """
  Documentation for `Day1`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Day1.hello()
      :world

  """

  def read(input) do
    input
    |> String.split("\n\n")
    |> Enum.map(fn line ->
      String.split(line) |> Enum.map(&Integer.parse(&1) |> elem(0))
    end)
  end

  def business_logic(input) do
    input
    |> Enum.map(&Enum.sum/1)
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.sum
  end

  def run(input) do
    input |> read() |> business_logic()
  end
end
