import { UserOperation, Comment, User, SortType, ListingType } from './interfaces';
import * as markdown_it from 'markdown-it';
import * as markdown_it_container from 'markdown-it-container';

export let repoUrl = 'https://github.com/dessalines/lemmy';

export function msgOp(msg: any): UserOperation {
  let opStr: string = msg.op;
  return UserOperation[opStr];
}

var md = new markdown_it({
  html: true,
  linkify: true,
  typographer: true
}).use(markdown_it_container, 'spoiler', {
  validate: function(params: any) {
    return params.trim().match(/^spoiler\s+(.*)$/);
  },

  render: function (tokens: any, idx: any) {
    var m = tokens[idx].info.trim().match(/^spoiler\s+(.*)$/);

    if (tokens[idx].nesting === 1) {
      // opening tag
      return '<details><summary>' + md.utils.escapeHtml(m[1]) + '</summary>\n';

    } else {
      // closing tag
      return '</details>\n';
    }
  }
});

export function hotRank(comment: Comment): number {
  // Rank = ScaleFactor * sign(Score) * log(1 + abs(Score)) / (Time + 2)^Gravity

  let date: Date = new Date(comment.published + 'Z'); // Add Z to convert from UTC date
  let now: Date = new Date();
  let hoursElapsed: number = (now.getTime() - date.getTime()) / 36e5;

  let rank = (10000 *  Math.log10(Math.max(1, 3 + comment.score))) / Math.pow(hoursElapsed + 2, 1.8);

  // console.log(`Comment: ${comment.content}\nRank: ${rank}\nScore: ${comment.score}\nHours: ${hoursElapsed}`);

  return rank;
}

export function mdToHtml(text: string) {
  return {__html: md.render(text)};
}

export function getUnixTime(text: string): number { 
  return text ? new Date(text).getTime()/1000 : undefined;
}

export function addTypeInfo<T>(arr: Array<T>, name: string): Array<{type_: string, data: T}> {  
  return arr.map(e => {return {type_: name, data: e}});
}

export function canMod(user: User, modIds: Array<number>, creator_id: number): boolean {
  // You can do moderator actions only on the mods added after you.
  if (user) {
    let yourIndex = modIds.findIndex(id => id == user.id);
    if (yourIndex == -1) {
      return false;
    } else { 
      modIds = modIds.slice(0, yourIndex+1); // +1 cause you cant mod yourself
      return !modIds.includes(creator_id);
    }
  } else {
    return false;
  }
}

export function isMod(modIds: Array<number>, creator_id: number): boolean {
  return modIds.includes(creator_id);
}


var imageRegex = new RegExp(`(http)?s?:?(\/\/[^"']*\.(?:png|jpg|jpeg|gif|png|svg))`);

export function isImage(url: string) {
  return imageRegex.test(url);
}

export let fetchLimit: number = 20;

export function capitalizeFirstLetter(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}


export function routeSortTypeToEnum(sort: string): SortType {
  if (sort == 'new') {
    return SortType.New;
  } else if (sort == 'hot') {
    return SortType.Hot;
  } else if (sort == 'topday') {
    return SortType.TopDay;
  } else if (sort == 'topweek') {
    return SortType.TopWeek;
  } else if (sort == 'topmonth') {
    return SortType.TopMonth;
  } else if (sort == 'topall') {
    return SortType.TopAll;
  }
}

export function routeListingTypeToEnum(type: string): ListingType {
  return ListingType[capitalizeFirstLetter(type)];
}

export async function getPageTitle(url: string) {
  let res = await fetch(`https://textance.herokuapp.com/title/${url}`);
  let data = await res.text();
  return data;
}

