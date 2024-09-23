/* eslint-disable */
import { DocumentTypeDecoration } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Commander = {
  __typename?: 'Commander';
  breakdownUrl: Scalars['String']['output'];
  colorId: Scalars['String']['output'];
  conversionRate: Scalars['Float']['output'];
  count: Scalars['Int']['output'];
  entries: Array<Entry>;
  id: Scalars['ID']['output'];
  imageUrls: Array<Scalars['String']['output']>;
  name: Scalars['String']['output'];
  topCuts: Scalars['Int']['output'];
  topEntries: Array<Entry>;
};


export type CommanderConversionRateArgs = {
  filters?: InputMaybe<CommanderStatsFilters>;
};


export type CommanderCountArgs = {
  filters?: InputMaybe<CommanderStatsFilters>;
};


export type CommanderEntriesArgs = {
  filters?: InputMaybe<EntryFilters>;
  sortBy?: InputMaybe<EntrySortBy>;
  sortDir?: InputMaybe<SortDirection>;
};


export type CommanderTopCutsArgs = {
  filters?: InputMaybe<CommanderStatsFilters>;
};


export type CommanderTopEntriesArgs = {
  sortBy?: InputMaybe<TopCommandersTopEntriesSortBy>;
  timePeriod?: InputMaybe<TimePeriod>;
};

export enum CommanderSortBy {
  Conversion = 'CONVERSION',
  Entries = 'ENTRIES',
  Name = 'NAME',
  TopCuts = 'TOP_CUTS'
}

export type CommanderStatsFilters = {
  colorId?: InputMaybe<Scalars['String']['input']>;
  maxDate?: InputMaybe<Scalars['String']['input']>;
  maxEntries?: InputMaybe<Scalars['Int']['input']>;
  maxSize?: InputMaybe<Scalars['Int']['input']>;
  minDate?: InputMaybe<Scalars['String']['input']>;
  minEntries?: InputMaybe<Scalars['Int']['input']>;
  minSize?: InputMaybe<Scalars['Int']['input']>;
  timePeriod?: InputMaybe<TimePeriod>;
  topCut?: InputMaybe<Scalars['Int']['input']>;
};

export type Entry = {
  __typename?: 'Entry';
  commander: Commander;
  decklist?: Maybe<Scalars['String']['output']>;
  draws: Scalars['Int']['output'];
  id: Scalars['ID']['output'];
  losses: Scalars['Int']['output'];
  lossesBracket: Scalars['Int']['output'];
  lossesSwiss: Scalars['Int']['output'];
  player?: Maybe<Player>;
  standing: Scalars['Int']['output'];
  tables: Array<TournamentTable>;
  tournament: Tournament;
  winRate?: Maybe<Scalars['Float']['output']>;
  wins: Scalars['Int']['output'];
  winsBracket: Scalars['Int']['output'];
  winsSwiss: Scalars['Int']['output'];
};

export type EntryFilters = {
  maxDate?: InputMaybe<Scalars['String']['input']>;
  maxDraws?: InputMaybe<Scalars['Int']['input']>;
  maxLosses?: InputMaybe<Scalars['Int']['input']>;
  maxSize?: InputMaybe<Scalars['Int']['input']>;
  maxStanding?: InputMaybe<Scalars['Int']['input']>;
  maxWins?: InputMaybe<Scalars['Int']['input']>;
  minDate?: InputMaybe<Scalars['String']['input']>;
  minDraws?: InputMaybe<Scalars['Int']['input']>;
  minLosses?: InputMaybe<Scalars['Int']['input']>;
  minSize?: InputMaybe<Scalars['Int']['input']>;
  minStanding?: InputMaybe<Scalars['Int']['input']>;
  minWins?: InputMaybe<Scalars['Int']['input']>;
};

export enum EntrySortBy {
  Date = 'DATE',
  Draws = 'DRAWS',
  Losses = 'LOSSES',
  Standing = 'STANDING',
  Winrate = 'WINRATE',
  Wins = 'WINS'
}

export type Player = {
  __typename?: 'Player';
  conversionRate: Scalars['Float']['output'];
  draws: Scalars['Int']['output'];
  entries: Array<Entry>;
  id: Scalars['ID']['output'];
  losses: Scalars['Int']['output'];
  name: Scalars['String']['output'];
  topCuts: Scalars['Int']['output'];
  topdeckProfile?: Maybe<Scalars['String']['output']>;
  winRate: Scalars['Float']['output'];
  wins: Scalars['Int']['output'];
};

export type Query = {
  __typename?: 'Query';
  commander: Commander;
  commanderNames: Array<Scalars['String']['output']>;
  commanders: Array<Commander>;
  player: Player;
  topCommanders: Array<Commander>;
  tournament: Tournament;
  tournaments: Array<Tournament>;
};


export type QueryCommanderArgs = {
  name: Scalars['String']['input'];
};


export type QueryCommandersArgs = {
  filters?: InputMaybe<CommanderStatsFilters>;
  sortBy?: InputMaybe<CommanderSortBy>;
  sortDir?: InputMaybe<SortDirection>;
};


export type QueryPlayerArgs = {
  profile: Scalars['String']['input'];
};


export type QueryTopCommandersArgs = {
  sortBy?: InputMaybe<TopCommandersSortBy>;
  timePeriod?: InputMaybe<TimePeriod>;
};


export type QueryTournamentArgs = {
  TID: Scalars['String']['input'];
};


export type QueryTournamentsArgs = {
  filters?: InputMaybe<TournamentFilters>;
  search?: InputMaybe<Scalars['String']['input']>;
  sortBy?: InputMaybe<TournamentSortBy>;
};

export enum SortDirection {
  Asc = 'ASC',
  Desc = 'DESC'
}

export enum TimePeriod {
  OneMonth = 'ONE_MONTH',
  SixMonths = 'SIX_MONTHS',
  ThreeMonths = 'THREE_MONTHS'
}

export enum TopCommandersSortBy {
  Conversion = 'CONVERSION',
  Popularity = 'POPULARITY'
}

export enum TopCommandersTopEntriesSortBy {
  New = 'NEW',
  Top = 'TOP'
}

export type Tournament = {
  __typename?: 'Tournament';
  TID: Scalars['String']['output'];
  entries: Array<Entry>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  rounds: Array<TournamentRound>;
  size: Scalars['Int']['output'];
  swissRounds: Scalars['Int']['output'];
  topCut: Scalars['Int']['output'];
  topPod: Array<Entry>;
  tournamentDate: Scalars['String']['output'];
};

export type TournamentFilters = {
  maxSize?: InputMaybe<Scalars['Int']['input']>;
  minSize?: InputMaybe<Scalars['Int']['input']>;
  timePeriod?: InputMaybe<TimePeriod>;
};

export type TournamentRound = {
  __typename?: 'TournamentRound';
  round: Scalars['String']['output'];
  tables: Array<TournamentTable>;
};

export enum TournamentSortBy {
  Date = 'DATE',
  Players = 'PLAYERS'
}

export type TournamentTable = {
  __typename?: 'TournamentTable';
  entries: Array<Maybe<Entry>>;
  roundName: Scalars['String']['output'];
  table: Scalars['Int']['output'];
  winner?: Maybe<Entry>;
  winnerSeatPosition?: Maybe<Scalars['Int']['output']>;
};

export type CommanderEntriesQueryVariables = Exact<{
  name: Scalars['String']['input'];
  filters?: InputMaybe<EntryFilters>;
}>;


export type CommanderEntriesQuery = { __typename?: 'Query', commander: { __typename?: 'Commander', entries: Array<{ __typename?: 'Entry', decklist?: string | null, winRate?: number | null }> } };

export type TopCommandersQueryVariables = Exact<{ [key: string]: never; }>;


export type TopCommandersQuery = { __typename?: 'Query', topCommanders: Array<{ __typename?: 'Commander', name: string }> };

export type LoadDataForCommanderQueryVariables = Exact<{
  name: Scalars['String']['input'];
  filters?: InputMaybe<EntryFilters>;
}>;


export type LoadDataForCommanderQuery = { __typename?: 'Query', commander: { __typename?: 'Commander', entries: Array<{ __typename?: 'Entry', decklist?: string | null, winRate?: number | null }> } };

export class TypedDocumentString<TResult, TVariables>
  extends String
  implements DocumentTypeDecoration<TResult, TVariables>
{
  __apiType?: DocumentTypeDecoration<TResult, TVariables>['__apiType'];

  constructor(private value: string, public __meta__?: Record<string, any>) {
    super(value);
  }

  toString(): string & DocumentTypeDecoration<TResult, TVariables> {
    return this.value;
  }
}

export const CommanderEntriesDocument = new TypedDocumentString(`
    query CommanderEntries($name: String!, $filters: EntryFilters) {
  commander(name: $name) {
    entries(filters: $filters) {
      decklist
      winRate
    }
  }
}
    `) as unknown as TypedDocumentString<CommanderEntriesQuery, CommanderEntriesQueryVariables>;
export const TopCommandersDocument = new TypedDocumentString(`
    query TopCommanders {
  topCommanders {
    name
  }
}
    `) as unknown as TypedDocumentString<TopCommandersQuery, TopCommandersQueryVariables>;
export const LoadDataForCommanderDocument = new TypedDocumentString(`
    query LoadDataForCommander($name: String!, $filters: EntryFilters) {
  commander(name: $name) {
    entries(filters: $filters) {
      decklist
      winRate
    }
  }
}
    `) as unknown as TypedDocumentString<LoadDataForCommanderQuery, LoadDataForCommanderQueryVariables>;