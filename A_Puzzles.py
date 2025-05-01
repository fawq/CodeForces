def main() -> None:
    number_of_children, number_of_puzzles = map(int, input().split())
    puzzles: list[int] = list(map(int, input().split()))
    puzzles.sort()

    min_distance = 1000
    for index in range(number_of_puzzles - number_of_children + 1):
        min_distance = min(min_distance, puzzles[index + number_of_children - 1] - puzzles[index])

    print(min_distance)

if __name__ == "__main__":
    main()