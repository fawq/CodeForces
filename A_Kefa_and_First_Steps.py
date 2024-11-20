def main() -> None:
    n: int = int(input())
    moneys: list[int] = list(map(int, input().split()))
    max_non_decreasing_subsequence: int = 1

    non_decreasing_subsequence: int = 1
    for index in range(1, n):
        if moneys[index] >= moneys[index - 1]:
            non_decreasing_subsequence += 1
            max_non_decreasing_subsequence = max(max_non_decreasing_subsequence, non_decreasing_subsequence)
        else:
            non_decreasing_subsequence = 1
    
    print(max_non_decreasing_subsequence)

if __name__ == "__main__":
    main()