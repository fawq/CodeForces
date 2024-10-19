def main():
    _ = int(input())
    coins: list[int] = sorted(map(int, input().split()), reverse=True)
    sum_of_all_coins: int = sum(coins)
    partial_sum: int = 0

    for index, coin in enumerate(coins):
        partial_sum += coin
        if partial_sum > sum_of_all_coins // 2:
            print(index + 1)
            break

if __name__ == "__main__":
    main()