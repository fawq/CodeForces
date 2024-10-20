def main() -> None:
    horseshoes: set[int] = set(map(int, input().split()))
    print(4 - len(horseshoes))

if __name__ == "__main__":
    main()