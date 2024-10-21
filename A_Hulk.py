def main() -> None:
    n: int = int(input())
    hate_loves: list[str] = []
    
    for index in range(n):
        if index %2 == 0:
            hate_loves.append("hate")
        else:
            hate_loves.append("love")

    print(f"I {' that I '.join(hate_loves)} it")

if __name__ == "__main__":
    main()