![IndraColor Logo](assets/logo.png)

![GitHub stars](https://img.shields.io/github/stars/seu-usuario/IndraColor?style=social)
![GitHub forks](https://img.shields.io/github/forks/seu-usuario/IndraColor?style=social)
![GitHub issues](https://img.shields.io/github/issues/seu-usuario/IndraColor)
![GitHub license](https://img.shields.io/github/license/seu-usuario/IndraColor)

> **IndraColor** é uma ferramenta de linha de comando que transforma suas imagens em paletas de cores harmoniosas e suaves. Inspirado no deus hindu Indra, conhecido por sua associação com o arco-íris e as cores, este projeto extrai automaticamente tons pastéis e cria combinações esteticamente agradáveis a partir de qualquer imagem.

## Características

- 🎨 Extração inteligente de cores dominantes
- 🎯 Geração de 3 tons da cor principal
- ✨ 2 cores de destaque complementares
- 🎭 Conversão automática para tons pastéis
- 🖥️ Interface de terminal intuitiva
- 🚀 Processamento rápido e eficiente
- 📦 Fácil instalação e uso

## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versão 1.70 ou superior)
- Terminal com suporte a cores (Windows Terminal, iTerm2, etc.)

## 🛠️ Instalação

1. Clone o repositório:
```bash
git clone https://github.com/VitorCarvalho67/IndraColor.git
cd IndraColor
```

2. Compile o projeto:
```bash
cargo build --release
```

3. (Opcional) Instale globalmente:
```bash
cargo install --path .
```

## 🚀 Como Usar

1. Execute o programa:
```bash
cargo run
```

2. Digite o caminho completo da imagem que deseja analisar
3. Pressione Enter para processar
4. Visualize as cores extraídas na interface
5. Pressione ESC para sair

### Exemplo de Uso
```bash
# Navegue até o diretório do projeto
cd IndraColor

# Execute o programa
cargo run

# Digite o caminho da imagem quando solicitado
C:\Users\Usuario\Imagens\minha_foto.jpg
```

## Sobre as Cores

O IndraColor processa as imagens da seguinte forma:

1. **Cor Principal**:
   - Extrai a cor dominante da imagem
   - Gera 3 variações:
     - Tom original
     - Versão mais clara
     - Versão mais escura

2. **Cores de Destaque**:
   - Seleciona 2 cores complementares
   - Mantém o equilíbrio visual

3. **Efeito Pastel**:
   - Aplica um filtro suavizante
   - Reduz a saturação em 30%
   - Aumenta o brilho em 30%

## 🤝 Contribuindo

Contribuições são bem-vindas! Siga estes passos:

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

### Padrões de Código

- Siga as convenções de nomenclatura do Rust
- Mantenha o código limpo e documentado
- Adicione testes para novas funcionalidades
- Atualize o README quando necessário

## 📝 Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Agradecimentos

- [color-thief](https://github.com/onur/color-thief) - Biblioteca para extração de cores
- [ratatui](https://github.com/tui-rs-revival/ratatui) - Framework para interfaces de terminal
- [image](https://github.com/image-rs/image) - Biblioteca para processamento de imagens

## Suporte

Encontrou um problema ou tem uma sugestão? Abra uma issue no GitHub!
