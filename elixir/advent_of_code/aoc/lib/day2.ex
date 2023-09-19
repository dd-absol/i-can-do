defmodule Day2 do
  def run(input) do
    parse(input)
    |> Enum.map(fn [a, b] -> result([a, b]) + value(b) end)
    |> Enum.sum
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&(~w/#{&1}/))
  end

  def result([x, y]) do
    a = conversion(x)
    case y do
      "Y" -> sign_value(a)
      "Z" -> a |> loses_to() |> sign_value()
      "X" -> a |> loses_to() |> loses_to() |> sign_value()
    end
  end

  def value(sign) do
    case sign do
      "X" -> 0
      "Y" -> 3
      "Z" -> 6
    end
  end

  def loses_to(sign) do
    case sign do
      :rock -> :paper
      :paper -> :scizor
      :scizor -> :rock
    end
  end

  def sign_value(sign) do
    case sign do
      :rock -> 1
      :paper -> 2
      :scizor -> 3
    end
  end

  def conversion(sign) do
    case sign do
      value when value in ["A", "X"] -> :rock
      value when value in ["B", "Y"] -> :paper
      value when value in ["C", "Z"] -> :scizor
    end
  end
end
