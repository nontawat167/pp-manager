import { IconBellRinging, IconReceipt2 } from '@tabler/icons-react';
import TestPage from '@Pages/TestPage/TestPage';
import TestPage2 from '@Pages/TestPage/TestPage2';
import { RouteData } from './routes';

export const routesList: RouteData[] = [
  {
    label: 'พิทักษ์พงศ์',
    title: 'พิทักษ์พงศ์ นะครัฟ',
    path: '/test',
    icon: IconBellRinging,
    component: TestPage,
  },
  {
    label: 'งามอ่อง',
    title: 'งามอ่อง อะดิ',
    path: '/test2',
    icon: IconReceipt2,
    component: TestPage,
  },
  {
    label: 'ล้อ',
    title: 'ล้อ title',
    path: '/wheel',
    icon: IconReceipt2,
    component: TestPage2,
  },
];
