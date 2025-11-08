#!/usr/bin/env python3
"""
Create a simple PDF for testing word position extraction.
Uses reportlab but with simpler compression to ensure lopdf compatibility.
"""

from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter

# Create PDF
pdf_path = "simple_v2.pdf"
c = canvas.Canvas(pdf_path, pagesize=letter)

# Set font
c.setFont("Helvetica", 12)

# Draw "Hello" at x=100, y=200 from top (592 from bottom for 792pt page height)
c.drawString(100, 592, "Hello")

# Draw "World" at x=160, y=200 from top (same baseline)
c.drawString(160, 592, "World")

# Save
c.save()

print(f"Created {pdf_path}")
print("Expected positions:")
print("  Hello: x=100, y=592, width=~27.3, height=12")
print("  World: x=160, y=592, width=~31.3, height=12")
