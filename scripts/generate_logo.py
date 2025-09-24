#!/usr/bin/env python3
"""
Script para gerar a logo do Activity Inquirer
Cria um ícone SVG e PNG com tema Gruvbox
"""

from pathlib import Path

def create_svg_logo():
    """Cria a logo em formato SVG"""
    svg_content = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="128" height="128" viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <!-- Gradiente Gruvbox -->
    <linearGradient id="bgGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#282828;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#3c3836;stop-opacity:1" />
    </linearGradient>
    
    <!-- Sombra -->
    <filter id="shadow" x="-50%" y="-50%" width="200%" height="200%">
      <feDropShadow dx="2" dy="2" stdDeviation="3" flood-color="#1d2021" flood-opacity="0.5"/>
    </filter>
  </defs>
  
  <!-- Fundo circular -->
  <circle cx="64" cy="64" r="60" fill="url(#bgGradient)" stroke="#504945" stroke-width="2" filter="url(#shadow)"/>
  
  <!-- Ícone de relógio/atividade -->
  <!-- Círculo do relógio -->
  <circle cx="64" cy="64" r="35" fill="none" stroke="#ebdbb2" stroke-width="3"/>
  
  <!-- Ponteiros do relógio -->
  <line x1="64" y1="64" x2="64" y2="40" stroke="#fabd2f" stroke-width="4" stroke-linecap="round"/>
  <line x1="64" y1="64" x2="80" y2="64" stroke="#fe8019" stroke-width="3" stroke-linecap="round"/>
  
  <!-- Centro do relógio -->
  <circle cx="64" cy="64" r="4" fill="#fb4934"/>
  
  <!-- Marcadores de hora -->
  <circle cx="64" cy="29" r="2" fill="#ebdbb2"/>
  <circle cx="99" cy="64" r="2" fill="#ebdbb2"/>
  <circle cx="64" cy="99" r="2" fill="#ebdbb2"/>
  <circle cx="29" cy="64" r="2" fill="#ebdbb2"/>
  
  <!-- Ícone de pergunta/inquérito -->
  <path d="M 45 20 Q 50 15 55 20 Q 60 25 55 30 L 52 35 M 52 40 L 52 42" 
        stroke="#8ec07c" stroke-width="2.5" fill="none" stroke-linecap="round"/>
  
  <!-- Ícone de lista/atividades -->
  <rect x="75" y="85" width="15" height="2" fill="#b8bb26" rx="1"/>
  <rect x="75" y="90" width="12" height="2" fill="#b8bb26" rx="1"/>
  <rect x="75" y="95" width="18" height="2" fill="#b8bb26" rx="1"/>
  
  <!-- Pontos indicadores -->
  <circle cx="72" cy="86" r="1.5" fill="#d3869b"/>
  <circle cx="72" cy="91" r="1.5" fill="#d3869b"/>
  <circle cx="72" cy="96" r="1.5" fill="#d3869b"/>
</svg>'''
    
    return svg_content

def create_png_simple(output_path, size=128):
    """Cria um PNG simples usando apenas PIL/Pillow"""
    try:
        from PIL import Image, ImageDraw

        # Cores Gruvbox
        bg_color = (40, 40, 40)  # #282828
        fg_color = (235, 219, 178)  # #ebdbb2
        accent1 = (250, 189, 47)  # #fabd2f (amarelo)
        accent2 = (254, 128, 25)  # #fe8019 (laranja)
        accent3 = (251, 73, 52)   # #fb4934 (vermelho)
        accent4 = (142, 192, 124) # #8ec07c (aqua)
        accent5 = (184, 187, 38)  # #b8bb26 (verde)

        # Criar imagem
        img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        # Fundo circular
        margin = size // 16
        draw.ellipse([margin, margin, size-margin, size-margin],
                    fill=bg_color, outline=(80, 73, 69), width=2)

        # Círculo do relógio
        center = size // 2
        clock_radius = size // 3
        draw.ellipse([center-clock_radius, center-clock_radius,
                     center+clock_radius, center+clock_radius],
                    outline=fg_color, width=3)

        # Ponteiros do relógio
        # Ponteiro das horas (amarelo)
        draw.line([center, center, center, center-clock_radius//2],
                 fill=accent1, width=4)
        # Ponteiro dos minutos (laranja)
        draw.line([center, center, center+clock_radius//2, center],
                 fill=accent2, width=3)

        # Centro do relógio
        center_size = size // 32
        draw.ellipse([center-center_size, center-center_size,
                     center+center_size, center+center_size],
                    fill=accent3)

        # Marcadores de hora
        marker_size = size // 64
        positions = [
            (center, center-clock_radius),  # 12h
            (center+clock_radius, center),  # 3h
            (center, center+clock_radius),  # 6h
            (center-clock_radius, center),  # 9h
        ]
        for x, y in positions:
            draw.ellipse([x-marker_size, y-marker_size,
                         x+marker_size, y+marker_size], fill=fg_color)

        # Ícone de pergunta (canto superior esquerdo)
        q_size = size // 8
        q_x, q_y = size//4, size//4
        draw.arc([q_x-q_size//2, q_y-q_size//2, q_x+q_size//2, q_y+q_size//2],
                start=0, end=180, fill=accent4, width=2)
        draw.ellipse([q_x-1, q_y+q_size//3-1, q_x+1, q_y+q_size//3+1], fill=accent4)

        # Ícone de lista (canto inferior direito)
        list_x = size * 3 // 4
        list_y = size * 3 // 4
        line_width = size // 16
        line_height = size // 64
        spacing = size // 32

        for i in range(3):
            y_pos = list_y + i * spacing
            # Bullet point
            draw.ellipse([list_x-line_width//2-spacing//2-1, y_pos-1,
                         list_x-line_width//2-spacing//2+1, y_pos+1], fill=accent5)
            # Line
            draw.rectangle([list_x-line_width//2, y_pos-line_height//2,
                           list_x+line_width//2, y_pos+line_height//2], fill=accent5)

        # Salvar
        img.save(output_path, 'PNG')
        return True

    except ImportError:
        print("PIL/Pillow não está disponível. Instale com: pip install Pillow")
        return False

def create_png_from_svg(svg_content, output_path, size=128):
    """Converte SVG para PNG usando cairosvg se disponível, senão usa método simples"""
    try:
        import cairosvg
        cairosvg.svg2png(bytestring=svg_content.encode('utf-8'),
                        write_to=str(output_path),
                        output_width=size,
                        output_height=size)
        return True
    except ImportError:
        # Fallback para método simples
        return create_png_simple(output_path, size)

def main():
    """Função principal"""
    # Criar diretório de assets se não existir
    assets_dir = Path(__file__).parent.parent / "assets"
    assets_dir.mkdir(exist_ok=True)
    
    # Gerar SVG
    svg_content = create_svg_logo()
    svg_path = assets_dir / "activity-inquirer.svg"
    
    with open(svg_path, 'w') as f:
        f.write(svg_content)
    
    print(f"✅ Logo SVG criada: {svg_path}")
    
    # Gerar PNG
    png_path = assets_dir / "activity-inquirer.png"
    if create_png_from_svg(svg_content, png_path):
        print(f"✅ Logo PNG criada: {png_path}")
    else:
        print("❌ Não foi possível criar o PNG. Apenas o SVG foi gerado.")
        print("Você pode converter manualmente usando um editor de imagens.")
    
    # Criar também ícones em diferentes tamanhos
    sizes = [16, 32, 48, 64, 128, 256]
    for size in sizes:
        size_png_path = assets_dir / f"activity-inquirer-{size}.png"
        if create_png_from_svg(svg_content, size_png_path, size):
            print(f"✅ Ícone {size}x{size} criado: {size_png_path}")

if __name__ == "__main__":
    main()
