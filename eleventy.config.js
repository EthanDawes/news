import mjml2html from 'mjml';
import { EleventyHtmlBasePlugin } from "@11ty/eleventy";

const config = {
  pathPrefix: "/news",
  dir: {
    input: "src",
  }
};

const origin = process.env.ORIGIN ?? "";

export default function(eleventyConfig) {
  // https://www.11ty.dev/docs/copy/
  eleventyConfig.addPassthroughCopy("src/**/*.{png,jpg}");

  // https://www.11ty.dev/docs/config/#configuration-api
  eleventyConfig.addTemplateFormats("mjml");

  // https://www.11ty.dev/docs/plugins/html-base/
  eleventyConfig.addPlugin(EleventyHtmlBasePlugin, {
		baseHref: origin + config.pathPrefix,
  });

  // https://www.11ty.dev/docs/languages/custom/
  eleventyConfig.addExtension("mjml", {
		compile: async (inputContent) => {
      // Removes liquid processing. Tried using transforms, but broke base plugin. Extensability apparently coming in future update
			const output = mjml2html(inputContent).html;

			return async () => {
				return output;
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