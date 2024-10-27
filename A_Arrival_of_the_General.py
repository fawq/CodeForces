def main() -> None:
    number = int(input())
    soldiers_height: list[int] = list(map(int, input().split()))
    max_height = 0
    max_index = 0
    min_height = 101
    min_index = 0

    for index, soldier_height in enumerate(soldiers_height):
        if soldier_height > max_height:
            max_height = soldier_height
            max_index = index
        if soldier_height <= min_height:
            min_height = soldier_height
            min_index = index
    
    if min_index > max_index: 
        print(max_index + number - min_index - 1)
    else:
        print(max_index + number - min_index - 2)

if __name__ == "__main__":
    main()