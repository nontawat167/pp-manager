import MainLayout from '@Components/layout/MainLayout';
import { MantineProvider } from '@mantine/core';
import { routesList } from './routes/routesList';

function App() {
  return (
    <MantineProvider
      theme={{ fontFamily: 'Prompt, sans-serif' }}
      withGlobalStyles
      withNormalizeCSS
    >
      <MainLayout initialRoutes={routesList} />
    </MantineProvider>
  );
}

export default App;
