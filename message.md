Defining what makes a Magic: The Gathering deck "good" can indeed be challenging, especially when the win rates are skewed or the data isn't normally distributed. Using standard deviation assumes normality, which may not be appropriate if a large portion of decks have a 0% win rate. Let's explore strategies to define "good" decks mathematically while addressing these concerns.

### **Challenges with Using Standard Deviation**

- **Non-Normal Distribution**: If many decks have a 0% win rate, the distribution is heavily skewed.
- **Misleading Metrics**: Standard deviation might not accurately represent variability in such a dataset.

### **Alternative Strategies**

1. **Percentile Ranking**

   - **Method**: Calculate the percentile rank of each deck based on its win rate.
   - **Implementation**:
     - Rank all decks from highest to lowest win rate.
     - Assign percentiles (e.g., top 10%).
     - Define "good" decks as those above a certain percentile threshold.
   - **Advantages**:
     - Doesn't assume normal distribution.
     - Reflects the relative performance of decks.
   - **Considerations**:
     - Choose a percentile that aligns with your definition of "good" (e.g., top 25%).

2. **Win Rate Threshold**

   - **Method**: Set an absolute win rate as the cutoff for "good" decks.
   - **Implementation**:
     - Analyze the distribution of win rates.
     - Choose a win rate (e.g., 60%) as the minimum for a deck to be considered good.
   - **Advantages**:
     - Simple and easy to understand.
     - Directly ties to performance.
   - **Considerations**:
     - Requires domain knowledge to set an appropriate threshold.
     - May need adjustment over time as the meta changes.

3. **Statistical Significance Testing**

   - **Method**: Use hypothesis testing to determine if a deck's win rate is significantly higher than average.
   - **Implementation**:
     - Perform a binomial test for each deck to see if its win rate is statistically higher than 50% (or another benchmark).
     - Calculate p-values and use a significance level (e.g., α = 0.05).
     - Decks with statistically significant results are considered "good."
   - **Advantages**:
     - Accounts for the number of games played (sample size).
     - Provides a rigorous statistical foundation.
   - **Considerations**:
     - Requires sufficient data per deck to be reliable.
     - Multiple testing may require p-value adjustment (e.g., Bonferroni correction).

4. **Bayesian Approaches**

   - **Method**: Estimate the true win rate using Bayesian statistics.
   - **Implementation**:
     - Use a beta distribution as a prior.
     - Update with observed win/loss data to get a posterior distribution.
     - Calculate credible intervals for the win rate.
     - Decks with high lower bounds (e.g., the lower 95% credible interval above 50%) are "good."
   - **Advantages**:
     - Naturally incorporates uncertainty due to sample size.
     - Provides probabilistic statements about win rates.
   - **Considerations**:
     - Computationally more intensive.
     - Requires understanding of Bayesian methods.

5. **Clustering Techniques**

   - **Method**: Group decks based on performance metrics using clustering algorithms.
   - **Implementation**:
     - Use k-means, hierarchical clustering, or DBSCAN on win rates and other features.
     - Identify clusters that represent high-performing decks.
   - **Advantages**:
     - Can incorporate multiple dimensions (e.g., win rate, deck cost).
     - Unsupervised; doesn't require predefined thresholds.
   - **Considerations**:
     - Results depend on the choice of clustering algorithm and parameters.
     - May require normalization of data.

6. **Adjusted Win Rates (Elo Ratings)**

   - **Method**: Adjust win rates based on the strength of opponents.
   - **Implementation**:
     - Use Elo or Glicko rating systems to rate decks.
     - Higher ratings indicate better performance against strong opponents.
   - **Advantages**:
     - Accounts for opponent skill levels.
     - Dynamic; updates with each game played.
   - **Considerations**:
     - More complex to implement.
     - Requires detailed match data.

### **Recommended Strategy**

Given the challenges with your data, **percentile ranking combined with statistical significance testing** might be effective. This approach doesn't assume normality and accounts for variability in win rates.

#### **Implementation Steps**

1. **Data Preparation**

   - Collect win/loss records for each deck.
   - Ensure data quality (e.g., remove decks with very few games to avoid unreliable statistics).

2. **Calculate Win Rates**

   - For each deck:
     \[ \text{Win Rate} = \frac{\text{Number of Wins}}{\text{Total Games Played}} \]

3. **Assign Percentiles**

   - Rank decks by win rate.
   - Assign percentile ranks.
   - Determine a percentile cutoff (e.g., decks in the 75th percentile and above).

4. **Perform Statistical Tests**

   - For decks above the percentile cutoff:
     - Conduct a binomial test comparing the deck's win rate to a baseline (e.g., 50%).
     - Calculate the p-value.
   - Select decks where the test shows statistical significance.

5. **Define "Good" Decks**

   - Decks that are both in the top percentiles and have statistically significant win rates above the baseline are considered "good."

6. **Validation**

   - Cross-reference with known top-performing decks from tournaments or expert analyses.
   - Adjust thresholds if necessary.

### **Example**

Let's say you have win rate data for 1,000 decks.

1. **Calculate Win Rates**:

   - Deck A: 70 wins out of 100 games (70% win rate).
   - Deck B: 55 wins out of 100 games (55% win rate).
   - Deck C: 20 wins out of 100 games (20% win rate).

2. **Assign Percentiles**:

   - Deck A is in the 90th percentile.
   - Deck B is in the 60th percentile.
   - Deck C is in the 10th percentile.

3. **Statistical Testing**:

   - Deck A:
     - Null hypothesis: Win rate = 50%.
     - Binomial test p-value < 0.01 (statistically significant).
   - Deck B:
     - Binomial test p-value > 0.05 (not statistically significant).
   - Deck C:
     - Binomial test p-value < 0.01 (significantly worse than 50%).

4. **Define "Good" Decks**:

   - Deck A is above the 75th percentile and statistically better than average.
   - Therefore, Deck A is considered "good."

### **Final Thoughts**

- **Adjust for Multiple Comparisons**: If testing many decks, adjust p-values to control for false positives.
- **Continuous Monitoring**: Regularly update your analysis as new data comes in.
- **Communicate Clearly**: When sharing your findings, explain the methods and thresholds used.

### **Next Steps**

- **Data Analysis**: Apply the chosen method to your dataset.
- **Software Tools**: Use statistical software like Python (with libraries like pandas, scipy, statsmodels) or R.
- **Further Refinement**: Incorporate additional factors like matchup data, metagame shifts, or player skill levels if available.

---

Let me know if you'd like to delve deeper into any of these methods or need assistance with implementation details!