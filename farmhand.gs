function FHPRICE(symbol, base = "usd") {
  var lowercaseSymbol = symbol.toLowerCase();

  var cache = CacheService.getScriptCache();
  var cached = cache.get(`price-${lowercaseSymbol}-${base}`);
  if (cached != null) {
    return cached;
  }

  var response = UrlFetchApp.fetch(`https://farmhand-xebhza4nba-ew.a.run.app/coin/${lowercaseSymbol}/price`);
  var price = JSON.parse(response.getContentText());

  cache.put(`price-${symbol}-usd`, price.usd, 60);
  cache.put(`price-${symbol}-btc`, price.btc, 60);
  cache.put(`price-${symbol}-eth`, price.eth, 60);

  return contents[base];
}

function FHCHANGE(symbol, daysAgo = 1, base = "usd") {
  var lowercaseSymbol = symbol.toLowerCase();

  var cache = CacheService.getScriptCache();
  var cached = cache.get(`pricechange-${lowercaseSymbol}-${base}`);
  if (cached != null) {
    return cached;
  }

  var response = UrlFetchApp.fetch(`https://farmhand-xebhza4nba-ew.a.run.app/coin/${lowercaseSymbol}/price-change/${daysAgo}`);
  var price = JSON.parse(response.getContentText());

  cache.put(`pricechange-${lowercaseSymbol}-usd`, price.usd, 60);
  cache.put(`pricechange-${lowercaseSymbol}-btc`, price.btc, 60);
  cache.put(`pricechange-${lowercaseSymbol}-eth`, price.eth, 60);

  return contents[base];
}
