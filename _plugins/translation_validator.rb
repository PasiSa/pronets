# frozen_string_literal: true

# Keep language routes and translation relationships unambiguous. This runs as
# part of every local and CI Jekyll build, before pages are rendered.
Jekyll::Hooks.register :site, :post_read do |site|
  documents = site.pages + site.collections.values.flat_map(&:docs)
  output_documents = documents.select { |document| !document.respond_to?(:output?) || document.output? }

  duplicate_urls = output_documents
                   .group_by(&:url)
                   .select { |url, grouped| !url.to_s.empty? && grouped.length > 1 }

  unless duplicate_urls.empty?
    details = duplicate_urls.map do |url, grouped|
      sources = grouped.map { |document| document.relative_path || document.path }.join(', ')
      "#{url}: #{sources}"
    end.join('; ')
    raise Jekyll::Errors::FatalException, "Duplicate generated URLs: #{details}"
  end

  translated_documents = output_documents.select { |document| document.data['translation_key'] }
  duplicate_translations = translated_documents
                           .group_by do |document|
                             [document.data['translation_key'], document.data['lang'] || site.config['lang']]
                           end
                           .select { |_identity, grouped| grouped.length > 1 }

  unless duplicate_translations.empty?
    details = duplicate_translations.map do |(key, lang), grouped|
      sources = grouped.map { |document| document.relative_path || document.path }.join(', ')
      "#{key} (#{lang}): #{sources}"
    end.join('; ')
    raise Jekyll::Errors::FatalException, "Duplicate translations: #{details}"
  end
end
