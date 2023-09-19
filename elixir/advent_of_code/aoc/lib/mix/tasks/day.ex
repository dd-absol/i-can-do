defmodule Mix.Tasks.Day do
  @moduledoc """
  cutom task to run each day automatically
  """
  defp parse_input(path) do
    with {:ok, content} <- File.read(path) do
      content
    end
  end

  def run_f(args, f, path_list) do
    module = String.to_atom("Elixir.Day" <> hd(args))
    path_list |> Enum.each(&(apply(module, :run, [parse_input("inputs/#{&1}/" <> hd(args) <> ".txt")]) |> f.()))
  end

  def run(args) do
    args |> run_f(&IO.inspect/1, ["small", "big"])
  end

  defmodule Puts do
    def run(args) do
      args |> Mix.Tasks.Day.run_f(&IO.puts/1, ["small", "big"])
    end
  end

  defmodule Small do
    def run(args) do
      args |> Mix.Tasks.Day.run_f(&IO.inspect/1, ["small"])
    end
  end
end
