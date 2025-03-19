# Sentivest

A powerful portfolio optimization utility that uses sentiment analysis on company and news data to minimize potential volatility.

## Description

Sentiment-Driven Portfolio Adjustments: The tool integrates sentiment analysis using FinBERT to evaluate the market sentiment for various stocks and companies, scraping news articles from sources like Yahoo Finance, Google News Scraper, and Twitter. This analysis feeds into the Black-Litterman model to dynamically adjust stock allocations and minimize portfolio risk based on real-time sentiment-driven views.

The tool combines Modern Portfolio Theory with the Black-Litterman model to optimize stock allocations. The Black-Litterman model refines MPT by incorporating subjective market views, ensuring a balanced, risk-averse portfolio. It recommends optimal stock holdings, reallocation strategies, and suggests diversification options to maximize returns while minimizing volatility.

NOTE: sentiment analysis for company data only works for tickers that reflect actual companies.

## TODOs
- [ ] Implement sentiment analysis module
- [ ] Develop Black-Litterman model integration
- [ ] Create portfolio optimization algorithm
- [ ] Design user interface for portfolio management
- [ ] Add database of stock data and cashe information that already is loaded
- [ ] Perform end-to-end testing
- [ ] Prepare documentation and user guide