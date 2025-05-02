def main() -> None:
    _ = input()
    events: list[int] = list(map(int, input().split()))
    
    event_sum: int = 0
    untreated_crimes: int = 0
    for event in events:
        if event > 0 and event_sum < 0:
            event_sum = 0
        event_sum += event

        if event_sum < 0:
            untreated_crimes += 1

    print(untreated_crimes)

if __name__ == "__main__":
    main()