def main() -> None:
    number_of_lines: int = int(input())
    occurences: dict[str, int] = {}

    for _ in range(number_of_lines):
        name: str = input()
        if name in occurences:
            print(f"{name}{occurences[name]}")
            occurences[name] += 1
        else:
            print("OK")
            occurences[name] = 1

if __name__ == "__main__":
    main()