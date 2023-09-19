defmodule Day7 do
  def run(input) do
    input
    |> parse()
    |> Enum.reduce(
      %{path: [], filesystem: %{}},
      &update_filesystem/2
    )
    |> Map.get(:filesystem)
    |> business_logic()
  end

  def parse(input) do
    input
    |> String.split("$", trim: true)
    |> Enum.map(&parse_command/1)
  end

  def parse_command(command) do
    case ~w/#{command}/ do
      ["cd", dir] -> {:cd, String.trim(dir)}
      _ -> {:ls, command |> String.split("\n", trim: true) |> tl |> Enum.map(&parse_item/1)}
    end
  end

  def parse_item(item) do
    case ~w/#{item}/ do
      ["dir", name] -> {:dir, name}
      [size, _] -> {:file, String.to_integer size}
    end
  end

  def update_filesystem(command, %{path: path, filesystem: filesystem}) do
    case command do
      {:cd, ".."} -> %{
        path: tl(path),
        filesystem: filesystem
      }
      {:cd, dir} -> %{
        path: [dir | path],
        filesystem: filesystem |> Map.put(
          stringpath([dir | path]),
          %{parent: path, children: [], size: 0}
        )
      }
      {:ls, items} -> %{
        path: path,
        filesystem: Map.update!(
          filesystem,
          stringpath(path),
          &(update_items(&1, items, path))
        )
      }
    end
  end

  def update_items(dir, items, path) do
    %{
      children: dir.children ++ (
        items
        |> Enum.filter(&(&1 |> elem(0) == :dir))
        |> Enum.map(fn {_, name} -> stringpath([name | path]) end)
      ),
      size: dir.size + (
        items
        |> Enum.filter(&(&1 |> elem(0) == :file))
        |> Enum.map(&(elem(&1, 1)))
        |> Enum.sum
      )
    }
  end

  def stringpath(path) do
    path
    |> Enum.reverse()
    |> Enum.join("/")
  end

  def actual_size(directory, filesystem) do
    directory.size + (
      directory.children
      |> Enum.map(&(actual_size(Map.get(filesystem, &1), filesystem)))
      |> Enum.sum
    )
  end

  def business_logic(filesystem) do
    filesystem
    |> Map.to_list()
    |> Enum.map(&(actual_size(&1 |> elem(1), filesystem)))
    |> part2
  end

  def part2(sizes) do
    sizes
    |> Enum.filter(&(&1 >= 30000000 - (70000000 - Enum.max(sizes))))
    |> Enum.min
  end
end
