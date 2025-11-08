#!/usr/bin/env python3
"""
Create a simple PDF with a table of contents for testing TOC extraction.
"""

from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter
from pypdf import PdfWriter, PdfReader
import io

# Create a multi-page PDF with content
buffer = io.BytesIO()
c = canvas.Canvas(buffer, pagesize=letter)

# Page 1: Introduction
c.setFont("Helvetica", 16)
c.drawString(100, 700, "Chapter 1: Introduction")
c.setFont("Helvetica", 12)
c.drawString(100, 650, "This is the introduction chapter.")
c.showPage()

# Page 2: Background
c.setFont("Helvetica", 16)
c.drawString(100, 700, "Chapter 2: Background")
c.setFont("Helvetica", 12)
c.drawString(100, 650, "This is the background chapter.")
c.showPage()

# Page 3: Methods
c.setFont("Helvetica", 16)
c.drawString(100, 700, "Chapter 3: Methods")
c.setFont("Helvetica", 12)
c.drawString(100, 650, "This is the methods chapter.")
c.drawString(100, 630, "Section 3.1: Data Collection")
c.showPage()

# Page 4: Results
c.setFont("Helvetica", 16)
c.drawString(100, 700, "Chapter 4: Results")
c.setFont("Helvetica", 12)
c.drawString(100, 650, "This is the results chapter.")
c.showPage()

c.save()

# Now add bookmarks/outline using PyPDF2
buffer.seek(0)
reader = PdfReader(buffer)
writer = PdfWriter()

# Add all pages
for page in reader.pages:
    writer.add_page(page)

# Add outline/bookmarks
parent_ch1 = writer.add_outline_item("Chapter 1: Introduction", 0)
parent_ch2 = writer.add_outline_item("Chapter 2: Background", 1)
parent_ch3 = writer.add_outline_item("Chapter 3: Methods", 2)
writer.add_outline_item("Section 3.1: Data Collection", 2, parent=parent_ch3)
parent_ch4 = writer.add_outline_item("Chapter 4: Results", 3)

# Write to file
with open("toc-test.pdf", "wb") as f:
    writer.write(f)

print("✅ Created toc-test.pdf with 4 chapters (Chapter 3 has 1 subsection)")
print("   Total outline entries: 5 (4 top-level + 1 nested)")
