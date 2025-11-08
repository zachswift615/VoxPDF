# Test Fixtures

## simple.pdf

**Created:** 2025-11-07 using Python reportlab library

**Specification:**
- Single page (US Letter: 612x792 points)
- Font: Helvetica 12pt
- Text: "Hello World"
- Two words positioned on the same line

**Word Positions:**

### "Hello"
- X: 100.0 points (from left edge)
- Y: 592.0 points (from bottom edge, 200 points from top)
- Width: 27.3 points (measured from font metrics)
- Height: 12.0 points (font size)

### "World"
- X: 160.0 points (from left edge)
- Y: 592.0 points (from bottom edge, 200 points from top)
- Width: 31.3 points (measured from font metrics)
- Height: 12.0 points (font size)

**Notes:**
- PDF coordinate system has origin at bottom-left
- Y-coordinate 592 corresponds to 200 points from top of page (792 - 592 = 200)
- Words are on the same line (same Y coordinate)
- Word spacing: 60 points horizontal separation between start positions

## simple.txt

Expected plain text extraction output for simple.pdf:
```
Hello World
```

## simple-positions.json

Expected word position data for simple.pdf. Contains an array of words with their bounding boxes.

Format:
```json
{
  "words": [
    {
      "text": "word text",
      "x": x-coordinate (float),
      "y": y-coordinate (float),
      "width": width in points (float),
      "height": height in points (float)
    }
  ]
}
```

## Future Fixtures (TODO)

### toc.pdf
- Multiple pages
- Has PDF outline with chapters/bookmarks
- For testing TOC extraction
