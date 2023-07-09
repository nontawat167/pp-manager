import moment from 'moment';

export const dateParse = (date: string, format: string) => {
  return moment(date).format(format);
};
