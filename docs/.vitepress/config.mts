import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "ASP CLI",
  description: "Datenkonvertierungs-Tool für JSON, YAML, TOML und CSV",
  lang: "de-DE",

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/getting-started" },
      { text: "Formate", link: "/formats/json" },
      { text: "Architektur", link: "/architecture/overview" },
      { text: "E-Learning", link: "/learning/introduction" },
    ],

    sidebar: {
      "/guide/": [
        {
          text: "Erste Schritte",
          items: [
            { text: "Einführung", link: "/guide/getting-started" },
            { text: "Installation", link: "/guide/installation" },
            { text: "Verwendung", link: "/guide/usage" },
            { text: "Beispiele", link: "/guide/examples" },
          ],
        },
      ],
      "/formats/": [
        {
          text: "Datenformate",
          items: [
            { text: "JSON", link: "/formats/json" },
            { text: "YAML", link: "/formats/yaml" },
            { text: "TOML", link: "/formats/toml" },
            { text: "CSV", link: "/formats/csv" },
            { text: "Vergleich", link: "/formats/comparison" },
          ],
        },
      ],
      "/architecture/": [
        {
          text: "Technische Dokumentation",
          items: [
            { text: "Überblick", link: "/architecture/overview" },
            { text: "Projektstruktur", link: "/architecture/structure" },
            { text: "Design-Entscheidungen", link: "/architecture/decisions" },
            { text: "Verwendete Libraries", link: "/architecture/libraries" },
          ],
        },
      ],
      "/learning/": [
        {
          text: "E-Learning",
          items: [
            { text: "Einführung", link: "/learning/introduction" },
            { text: "Lernziele", link: "/learning/objectives" },
            { text: "Entwicklungsprozess", link: "/learning/development" },
            { text: "Parsing-Strategien", link: "/learning/parsing" },
            { text: "Fehlerbehandlung", link: "/learning/error-handling" },
            { text: "Erkenntnisse", link: "/learning/insights" },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/Arda450/cli-data-converter" },
    ],

    footer: {
      message: "Advanced Specialised Project - Datenkonvertierung mit CLI",
      copyright: "Copyright © 2025 Arda Karadavut",
    },

    search: {
      provider: "local",
    },
  },
});
