def main() -> None:
    number_of_drinks: int = int(input())
    sum_of_percentages: int = sum(map(int, input().split()))

    # It will not cover the same output but should be in bound
    print(sum_of_percentages / number_of_drinks)

if __name__ == "__main__":
    main()