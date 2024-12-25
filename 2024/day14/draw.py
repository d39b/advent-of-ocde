from PIL import Image, ImageColor

def main():
    with open('out.txt') as f:
        i = 1;
        for line in f:
            im = Image.new("1", (101, 103))
            positions = line.strip().split(" ")
            for j in range(0, len(positions), 2):
                x = int(positions[j])
                y = int(positions[j+1])
                im.putpixel((x, y), ImageColor.getcolor('white', '1'))

            im.save(f"images/image-{i}.png")
            i += 1

    pass

if __name__ == "__main__":
    main()
