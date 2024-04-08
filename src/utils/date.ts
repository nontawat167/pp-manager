import day from 'dayjs';
import LocalizedFormat from 'dayjs/plugin/localizedFormat';

export const dateParse = (date: string, format: string) => {
  day.extend(LocalizedFormat);
  return day(date).format(format);
};
