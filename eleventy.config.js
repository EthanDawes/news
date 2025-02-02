import mjml2html from 'mjml';
import { EleventyHtmlBasePlugin, EleventyRenderPlugin } from "@11ty/eleventy";

const config = {
  pathPrefix: "/news",
  dir: {
    input: "src",
  }
};

const origin = process.env.ORIGIN ?? "";

export default function(eleventyConfig) {
  // https://www.11ty.dev/docs/data-global-custom/
  eleventyConfig.addGlobalData("server", process.env.CI ? "https://news.ethandawes.dev" : "about:blank");

  // https://www.11ty.dev/docs/copy/
  eleventyConfig.addPassthroughCopy("src/**/*.{png,jpg}");

  // https://www.11ty.dev/docs/config/#configuration-api
  eleventyConfig.addTemplateFormats("mjml");

  // https://www.11ty.dev/docs/plugins/html-base/
  eleventyConfig.addPlugin(EleventyHtmlBasePlugin, {
		baseHref: origin + config.pathPrefix,
  });

  // https://www.11ty.dev/docs/plugins/render/
  eleventyConfig.addPlugin(EleventyRenderPlugin);

  // https://www.11ty.dev/docs/languages/custom/
  eleventyConfig.addExtension("mjml", {
    key: "liquid",
		compile: async function() {
      return async (data) => {
        const content = await this.defaultRenderer(data);
        // Using `render` tag for components: https://shopify.dev/docs/api/liquid/tags/render
        return mjml2html(content).html;
			};
		},
	});

  // https://www.11ty.dev/docs/collections-api/#getfilteredbyglob(-glob-)
  eleventyConfig.addCollection("posts", function (collectionApi) {
		return collectionApi.getFilteredByGlob("src/[0-9][0-9][0-9][0-9]/**");
	});

  eleventyConfig.addGlobalData("eleventyComputed", {
    permalink: (data) => {
      if (data.page.inputPath.match(/[0-9]{4}/)) {
        // Permalink format for posts: /<year>/<short month>/
        const year = new Intl.DateTimeFormat('en', { year: 'numeric' }).format(data.page.date);
        const month = new Intl.DateTimeFormat('en', { month: 'short' }).format(data.page.date);
        return `/${year}/${month}/`;
      }
      return data.permalink; // Preserve any existing permalink or default
    },
  });

  return config;
};